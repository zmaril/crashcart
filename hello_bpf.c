// hello_bpf.c

#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

SEC("xdp")
int hello_world(struct xdp_md *ctx) {
    bpf_printk("Hello, World!\n");
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
