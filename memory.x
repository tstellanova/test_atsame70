/* Memory layout for Atmel SAM E70q21b */
MEMORY
{


  /* FLASH and RAM are mandatory memory regions */
  FLASH (rx)  : ORIGIN = 0x00400000, LENGTH = 2048K
  RAM_DTCM    : ORIGIN = 0x20000000, LENGTH = 16K
  RAM (rwx)   : ORIGIN = 0x20400000, LENGTH = 384K

}

/* The location of the stack can be overridden using the
   `_stack_start` symbol.  Place the stack at the end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);


/*
In case of SAM E70,
16KB of Instruction Cache and
16KB of data Cache is available.
*/
