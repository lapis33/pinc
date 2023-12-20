.global entry

entry:
    ldr sp, =stack_top
    bl kmain
    b .
