ENTRY(bootstrap_start)
OUTPUT_FORMAT(elf32-powerpc)

PHDRS
{
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
    bss PT_LOAD FLAGS(6);
}

SECTIONS {
    . = 0x80003100;

    .text ALIGN(0x10) : {
        KEEP (*(.text.crt0))
        *(.text .text.*)
    } :text

    .data ALIGN(0x10) : {
        *(.data .data.*)
    } : data

    .rodata ALIGN(0x10) : {
        *(.rodata .rodata.*)
    } : data

    .got ALIGN(0x10) : {
        *(.got)
    } : data

    .data.rel.ro ALIGN(0x10) : {
        *(.data.rel.ro.local.*) *(data.rel.ro .data.rel.ro.*)
    } : data

    .bss ALIGN(0x10) : {
        _bss_bottom = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_top = .;

        . = ALIGN(16);
        PROVIDE(_stack_bottom = .);
        . = . + 0x10000; /* 64kB of stack memory */
        . = ALIGN(16);
        PROVIDE(_stack_top = .);
    } :bss

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
    }
}