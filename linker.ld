ENTRY(_entry)

SECTIONS {
    . = 0x40000000;

    . += 0x10000;
    stack_top = .;
}
