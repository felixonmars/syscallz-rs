/// An enum of all syscalls
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Syscall {
restart_syscall = 0,
exit = 1,
fork = 2,
read = 3,
write = 4,
open = 5,
close = 6,
wait4 = 7,
creat = 8,
link = 9,
unlink = 10,
execv = 11,
chdir = 12,
chown = 13,
mknod = 14,
chmod = 15,
lchown = 16,
brk = 17,
perfctr = 18,
lseek = 19,
getpid = 20,
capget = 21,
capset = 22,
setuid = 23,
getuid = 24,
vmsplice = 25,
ptrace = 26,
alarm = 27,
sigaltstack = 28,
pause = 29,
utime = 30,
lchown32 = 31,
fchown32 = 32,
access = 33,
nice = 34,
chown32 = 35,
sync = 36,
kill = 37,
stat = 38,
sendfile = 39,
lstat = 40,
dup = 41,
pipe = 42,
times = 43,
getuid32 = 44,
umount2 = 45,
setgid = 46,
getgid = 47,
signal = 48,
geteuid = 49,
getegid = 50,
acct = 51,
getgid32 = 53,
ioctl = 54,
reboot = 55,
mmap2 = 56,
symlink = 57,
readlink = 58,
execve = 59,
umask = 60,
chroot = 61,
fstat = 62,
fstat64 = 63,
getpagesize = 64,
msync = 65,
vfork = 66,
pread64 = 67,
pwrite64 = 68,
geteuid32 = 69,
getegid32 = 70,
mmap = 71,
setreuid32 = 72,
munmap = 73,
mprotect = 74,
madvise = 75,
vhangup = 76,
truncate64 = 77,
mincore = 78,
getgroups = 79,
setgroups = 80,
getpgrp = 81,
setgroups32 = 82,
setitimer = 83,
ftruncate64 = 84,
swapon = 85,
getitimer = 86,
setuid32 = 87,
sethostname = 88,
setgid32 = 89,
dup2 = 90,
setfsuid32 = 91,
fcntl = 92,
select = 93,
setfsgid32 = 94,
fsync = 95,
setpriority = 96,
socket = 97,
connect = 98,
accept = 99,
getpriority = 100,
rt_sigreturn = 101,
rt_sigaction = 102,
rt_sigprocmask = 103,
rt_sigpending = 104,
rt_sigtimedwait = 105,
rt_sigqueueinfo = 106,
rt_sigsuspend = 107,
setresuid32 = 108,
getresuid32 = 109,
setresgid32 = 110,
getresgid32 = 111,
setregid32 = 112,
recvmsg = 113,
sendmsg = 114,
getgroups32 = 115,
gettimeofday = 116,
getrusage = 117,
getsockopt = 118,
getcwd = 119,
readv = 120,
writev = 121,
settimeofday = 122,
fchown = 123,
fchmod = 124,
recvfrom = 125,
setreuid = 126,
setregid = 127,
rename = 128,
truncate = 129,
ftruncate = 130,
flock = 131,
lstat64 = 132,
sendto = 133,
shutdown = 134,
socketpair = 135,
mkdir = 136,
rmdir = 137,
utimes = 138,
stat64 = 139,
sendfile64 = 140,
getpeername = 141,
futex = 142,
gettid = 143,
getrlimit = 144,
setrlimit = 145,
pivot_root = 146,
prctl = 147,
pciconfig_read = 148,
pciconfig_write = 149,
getsockname = 150,
inotify_init = 151,
inotify_add_watch = 152,
poll = 153,
getdents64 = 154,
fcntl64 = 155,
inotify_rm_watch = 156,
statfs = 157,
fstatfs = 158,
umount = 159,
sched_set_affinity = 160,
sched_get_affinity = 161,
getdomainname = 162,
setdomainname = 163,
quotactl = 165,
set_tid_address = 166,
mount = 167,
ustat = 168,
setxattr = 169,
lsetxattr = 170,
fsetxattr = 171,
getxattr = 172,
lgetxattr = 173,
getdents = 174,
setsid = 175,
fchdir = 176,
fgetxattr = 177,
listxattr = 178,
llistxattr = 179,
flistxattr = 180,
removexattr = 181,
lremovexattr = 182,
sigpending = 183,
query_module = 184,
setpgid = 185,
fremovexattr = 186,
tkill = 187,
exit_group = 188,
uname = 189,
init_module = 190,
personality = 191,
remap_file_pages = 192,
epoll_create = 193,
epoll_ctl = 194,
epoll_wait = 195,
ioprio_set = 196,
getppid = 197,
sigaction = 198,
sgetmask = 199,
ssetmask = 200,
sigsuspend = 201,
oldlstat = 202,
uselib = 203,
readdir = 204,
readahead = 205,
socketcall = 206,
syslog = 207,
lookup_dcookie = 208,
fadvise64 = 209,
fadvise64_64 = 210,
tgkill = 211,
waitpid = 212,
swapoff = 213,
sysinfo = 214,
ipc = 215,
sigreturn = 216,
clone = 217,
ioprio_get = 218,
adjtimex = 219,
sigprocmask = 220,
create_module = 221,
delete_module = 222,
get_kernel_syms = 223,
getpgid = 224,
bdflush = 225,
sysfs = 226,
afs_syscall = 227,
setfsuid = 228,
setfsgid = 229,
_newselect = 230,
time = 231,
splice = 232,
stime = 233,
statfs64 = 234,
fstatfs64 = 235,
_llseek = 236,
mlock = 237,
munlock = 238,
mlockall = 239,
munlockall = 240,
sched_setparam = 241,
sched_getparam = 242,
sched_setscheduler = 243,
sched_getscheduler = 244,
sched_yield = 245,
sched_get_priority_max = 246,
sched_get_priority_min = 247,
sched_rr_get_interval = 248,
nanosleep = 249,
mremap = 250,
_sysctl = 251,
getsid = 252,
fdatasync = 253,
nfsservctl = 254,
sync_file_range = 255,
clock_settime = 256,
clock_gettime = 257,
clock_getres = 258,
clock_nanosleep = 259,
sched_getaffinity = 260,
sched_setaffinity = 261,
timer_settime = 262,
timer_gettime = 263,
timer_getoverrun = 264,
timer_delete = 265,
timer_create = 266,
vserver = 267,
io_setup = 268,
io_destroy = 269,
io_submit = 270,
io_cancel = 271,
io_getevents = 272,
mq_open = 273,
mq_unlink = 274,
mq_timedsend = 275,
mq_timedreceive = 276,
mq_notify = 277,
mq_getsetattr = 278,
waitid = 279,
tee = 280,
add_key = 281,
request_key = 282,
keyctl = 283,
openat = 284,
mkdirat = 285,
mknodat = 286,
fchownat = 287,
futimesat = 288,
fstatat64 = 289,
unlinkat = 290,
renameat = 291,
linkat = 292,
symlinkat = 293,
readlinkat = 294,
fchmodat = 295,
faccessat = 296,
pselect6 = 297,
ppoll = 298,
unshare = 299,
set_robust_list = 300,
get_robust_list = 301,
migrate_pages = 302,
mbind = 303,
get_mempolicy = 304,
set_mempolicy = 305,
kexec_load = 306,
move_pages = 307,
getcpu = 308,
epoll_pwait = 309,
utimensat = 310,
signalfd = 311,
timerfd_create = 312,
eventfd = 313,
fallocate = 314,
timerfd_settime = 315,
timerfd_gettime = 316,
signalfd4 = 317,
eventfd2 = 318,
epoll_create1 = 319,
dup3 = 320,
pipe2 = 321,
inotify_init1 = 322,
accept4 = 323,
preadv = 324,
pwritev = 325,
rt_tgsigqueueinfo = 326,
perf_event_open = 327,
recvmmsg = 328,
fanotify_init = 329,
fanotify_mark = 330,
prlimit64 = 331,
name_to_handle_at = 332,
open_by_handle_at = 333,
clock_adjtime = 334,
syncfs = 335,
sendmmsg = 336,
setns = 337,
process_vm_readv = 338,
process_vm_writev = 339,
kern_features = 340,
kcmp = 341,
finit_module = 342,
sched_setattr = 343,
sched_getattr = 344,
renameat2 = 345,
seccomp = 346,
getrandom = 347,
memfd_create = 348,
bpf = 349,
execveat = 350,
membarrier = 351,
userfaultfd = 352,
bind = 353,
listen = 354,
setsockopt = 355,
mlock2 = 356,
copy_file_range = 357,
preadv2 = 358,
pwritev2 = 359,
statx = 360,
io_pgetevents = 361,
pkey_mprotect = 362,
pkey_alloc = 363,
pkey_free = 364,
rseq = 365,
semget = 393,
semctl = 394,
shmget = 395,
shmctl = 396,
shmat = 397,
shmdt = 398,
msgget = 399,
msgsnd = 400,
msgrcv = 401,
msgctl = 402,
clock_gettime64 = 403,
clock_settime64 = 404,
clock_adjtime64 = 405,
clock_getres_time64 = 406,
clock_nanosleep_time64 = 407,
timer_gettime64 = 408,
timer_settime64 = 409,
timerfd_gettime64 = 410,
timerfd_settime64 = 411,
utimensat_time64 = 412,
pselect6_time64 = 413,
ppoll_time64 = 414,
io_pgetevents_time64 = 416,
recvmmsg_time64 = 417,
mq_timedsend_time64 = 418,
mq_timedreceive_time64 = 419,
semtimedop_time64 = 420,
rt_sigtimedwait_time64 = 421,
futex_time64 = 422,
sched_rr_get_interval_time64 = 423,
pidfd_send_signal = 424,
io_uring_setup = 425,
io_uring_enter = 426,
io_uring_register = 427,
open_tree = 428,
move_mount = 429,
fsopen = 430,
fsconfig = 431,
fsmount = 432,
fspick = 433,
pidfd_open = 434,
close_range = 436,
openat2 = 437,
pidfd_getfd = 438,
faccessat2 = 439,
process_madvise = 440,
epoll_pwait2 = 441,
mount_setattr = 442,
landlock_create_ruleset = 444,
landlock_add_rule = 445,
landlock_restrict_self = 446,
}
