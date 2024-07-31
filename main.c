// main.c

#include <stdio.h>
#include <stdlib.h>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>

int main() {
    struct bpf_object *obj;
    int prog_fd;
    
    obj = bpf_object__open_file("hello_bpf.o", NULL);
    if (libbpf_get_error(obj)) {
        fprintf(stderr, "ERROR: opening BPF object file failed\n");
        return 1;
    }

    if (bpf_object__load(obj)) {
        fprintf(stderr, "ERROR: loading BPF object file failed\n");
        return 1;
    }

    prog_fd = bpf_program__fd(bpf_object__find_program_by_name(obj, "hello_world"));
    if (prog_fd < 0) {
        fprintf(stderr, "ERROR: finding BPF program in object file failed\n");
        return 1;
    }

    printf("Hello, World BPF program loaded successfully!\n");
    bpf_object__close(obj);

    return 0;
}
