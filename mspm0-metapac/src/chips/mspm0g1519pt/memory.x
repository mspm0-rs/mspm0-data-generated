MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 512K
    RAM : ORIGIN = 0x20200000, LENGTH = 64K
    # Note: RAM on this device is continuous memory but partitioned in the
    # linker into two separate sections. This is to account for the upper 64kB
    # of RAM being wiped out upon the device entering any low-power mode
    # stronger than SLEEP. Thus, it is up to the end-user to enable RAM_BANK for
    # applications where the memory is considered lost outside of RUN and SLEEP Modes.
    RAM_BANK : ORIGIN = 0x20210000, LENGTH = 64K
}