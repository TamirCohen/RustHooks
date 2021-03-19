MEMORY
{
  CODE : ORIGIN = 0x20007000, LENGTH = 5K
}

/* TODO: Should delete in future - we need to tell the linker to keep all pubs */
EXTERN(nothing);

SECTIONS
{
   .text ORIGIN(CODE):
   {
       *(.text .text.*);
   } > CODE

   .rodata :
   {
       *(.rodata .rodata.*);
   } > CODE

   /DISCARD/ :
   {
/* TODO: REMOVE all the other not nesecerry sections*/

      *(.ARM.exidx .ARM.exidx.*);
   }
}
