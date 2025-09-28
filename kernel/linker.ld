ENTRY(_start)

SECTIONS
{
    . = 0x40080000; /* Load address for ARM64 */
    
    .text : {
        KEEP(*(.text.boot))
        KEEP(*(.text.exceptions))
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }
    
    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.build-id)
    }
}