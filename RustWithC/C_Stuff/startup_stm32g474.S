.syntax unified
.cpu cortex-m4
.thumb

/* Linker-Symbole */
.extern _estack
.extern _sidata
.extern _sdata
.extern _edata
.extern _sbss
.extern _ebss

.global Reset_Handler
.type   Reset_Handler, %function
.thumb_func
Reset_Handler:
  /* Copy .data from FLASH to RAM */
  ldr r0, =_sidata
  ldr r1, =_sdata
  ldr r2, =_edata
1:
  cmp r1, r2
  bhs 2f
  ldr r3, [r0], #4
  str r3, [r1], #4
  b 1b

2:
  /* Zero .bss */
  ldr r1, =_sbss
  ldr r2, =_ebss
  movs r3, #0
3:
  cmp r1, r2
  bhs 4f
  str r3, [r1], #4
  b 3b

4:
  bl main
5:
  b 5b

.global Default_Handler
.type   Default_Handler, %function
.thumb_func
Default_Handler:
  b Default_Handler

/* --- Vector Table --- */
.section .isr_vector, "a", %progbits
  .word _estack
  .word Reset_Handler
  /* minimal: just core exceptions; rest to Default_Handler */
  .word Default_Handler /* NMI */
  .word Default_Handler /* HardFault */
  .word Default_Handler /* MemManage */
  .word Default_Handler /* BusFault */
  .word Default_Handler /* UsageFault */
  .word 0
  .word 0
  .word 0
  .word 0
  .word Default_Handler /* SVC */
  .word Default_Handler /* DebugMon */
  .word 0
  .word Default_Handler /* PendSV */
  .word Default_Handler /* SysTick */
