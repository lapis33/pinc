.global _entry

_entry:
    ldr sp, =stack_top
    bl kmain
    b .
