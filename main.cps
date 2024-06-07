/*
 *
 * kernel.c - version 0.0.1
 * https://github.com/chipsetx/Simple-Kernel-in-C-and-Assembly/blob/master/kernel.c
 */
#define WHITE_TXT 0x07 /* light gray on black text */
/* simple kernel written in C */
void k_main() 
{
	k_clear_screen();
	k_printf("Hello, world! Welcome to my kernel.", 0);
};

/* k_clear_screen : to clear the entire text screen */
void k_clear_screen()
{
	char *vidmem = (char *) 0xb8000;
	unsigned int i=0;
	while(i < (80*25*2))
	{
		vidmem[i]=' ';
		i++;
		vidmem[i]=WHITE_TXT;
		i++;
	};
};

/* k_printf : the message and the line # */
unsigned int k_printf(char *message, unsigned int line)
{
	char *vidmem = (char *) 0xb8000;
	unsigned int i=0;
	i=(line*80*2);

	while(*message!=0)
	{
		if(*message=='\n') // check for a new line
		{
			line++;
			i=(line*80*2);
			*message++;
		} else {
			vidmem[i]=*message;
			*message++;
			i++;
			vidmem[i]=WHITE_TXT;
			i++;
		};

	};

	return(1);
}
/* k_strcpy : copy a string */
void k_strcpy(char *dest, char *src) {
    while (*src != '\0') {
        *dest++ = *src++;
    }
    *dest = '\0';
}