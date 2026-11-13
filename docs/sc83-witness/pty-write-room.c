/* Black-box Linux pty/poll/epoll probe; no implementation source input.
 * Match write_partial's ICANON/ECHO-off setup, fill/refill until quiet,
 * then report the kernel's answers after every bounded slave read.
 * Usage: probe READ_SIZE (1..4096) [WRITE_DELAY_US (0..1000)].
 * No EPOLLET/ONESHOT or re-arming. The optional delay models time between
 * write_some calls without changing the bytes, flags or registration.
 */
#define _XOPEN_SOURCE 600
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/epoll.h>
#include <sys/ioctl.h>
#include <sys/utsname.h>
#include <termios.h>
#include <unistd.h>

static void die(const char *s) { perror(s); exit(2); }
static int look(int ep, int ms) {
    struct epoll_event ev;
    int n = epoll_wait(ep, &ev, 1, ms);
    if (n < 0) die("epoll_wait");
    return n ? (int)ev.events : 0;
}
static unsigned write_delay;
static long fill(int m) {
    char b[4096]; memset(b, 'x', sizeof b);
    long total = 0;
    for (int i = 0; i < 1000; ++i) {
        ssize_t n = write(m, b, sizeof b);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return total;
            die("write");
        }
        total += n;
        if (write_delay) usleep(write_delay);
    }
    fprintf(stderr, "fill bound reached\n"); exit(3);
}
int main(int argc, char **argv) {
    int chunk = argc >= 2 ? atoi(argv[1]) : 4096;
    write_delay = argc >= 3 ? (unsigned)atoi(argv[2]) : 0;
    if (chunk < 1 || chunk > 4096 || write_delay > 1000) return 2;
    setvbuf(stdout, NULL, _IOLBF, 0);
    struct utsname u; if (uname(&u)) die("uname");
    printf("kernel=%s %s chunk=%d write_delay_us=%u\n", u.sysname, u.release, chunk, write_delay);
    int m = posix_openpt(O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (m < 0 || grantpt(m) || unlockpt(m)) die("open master");
    char *name = ptsname(m); if (!name) die("ptsname");
    int s = open(name, O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (s < 0) die("open slave");
    struct termios t; if (tcgetattr(s, &t)) die("tcgetattr");
    t.c_lflag &= ~(ICANON | ECHO);
    if (tcsetattr(s, TCSANOW, &t)) die("tcsetattr");
    printf("iflag=%lx oflag=%lx lflag=%lx vmin=%u vtime=%u\n",
           (unsigned long)t.c_iflag, (unsigned long)t.c_oflag,
           (unsigned long)t.c_lflag, t.c_cc[VMIN], t.c_cc[VTIME]);
    int ep = epoll_create1(EPOLL_CLOEXEC); if (ep < 0) die("epoll_create1");
    struct epoll_event add = { .events = EPOLLOUT, .data.fd = m };
    if (epoll_ctl(ep, EPOLL_CTL_ADD, m, &add)) die("epoll_ctl");
    printf("initial_epoll=%x\n", look(ep, 0));
    long sent = fill(m);
    int full = look(ep, 25), settles = 0;
    while (full && settles++ < 128) { sent += fill(m); full = look(ep, 25); }
    printf("sent=%ld settles=%d full_epoll=%x\n", sent, settles, full);
    if (full) return 3;
    char b[4096]; long drained = 0;
    int room = 0;
    for (int i = 1; i <= 65536 && drained < sent; ++i) {
        ssize_t n = read(s, b, (size_t)chunk);
        if (n < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            struct pollfd in = { .fd = s, .events = POLLIN };
            if (poll(&in, 1, 2000) <= 0) die("slave readiness");
            n = read(s, b, (size_t)chunk);
        }
        if (n <= 0) die("read");
        drained += n;
        int queued = -1; if (ioctl(s, TIOCINQ, &queued)) die("TIOCINQ");
        struct pollfd out = { .fd = m, .events = POLLOUT };
        if (poll(&out, 1, 0) < 0) die("poll");
        int immediate = look(ep, 0);
        /* Match the row's original two-second post-read wait once. */
        int waited = look(ep, i == 1 ? 2000 : 0);
        room = !!((immediate | waited) & EPOLLOUT);
        printf("read=%d bytes=%zd drained=%ld inq=%d poll=%x epoll0=%x epollwait=%x\n",
               i, n, drained, queued, out.revents, immediate, waited);
        if (room) {
            printf("level_repeat=%x,%x\n", look(ep, 0), look(ep, 0));
            break;
        }
    }
    close(ep); close(s); close(m);
    printf("room=%d drained=%ld\n", room, drained);
    return room ? 0 : 1;
}
