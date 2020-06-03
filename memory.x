/* Memory layout for Atmel SAM E70q21b */
MEMORY
{


  /* TODO verify these memory regions and length in K */
  /* FLASH and RAM are mandatory memory regions */
  FLASH  : ORIGIN = 0x00400000, LENGTH = 1024K
  RAM    : ORIGIN = 0x20000000, LENGTH = 384K

}

/* The location of the stack can be overridden using the
   `_stack_start` symbol.  Place the stack at the end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* The location of the .text section can be overridden using the
   `_stext` symbol.  By default it will place after .vector_table */
/* _stext = ORIGIN(FLASH) + 0x40c; */

