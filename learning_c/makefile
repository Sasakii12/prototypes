CC	=	gcc
CFLAGS	=	-g
RM	=	rm	-f


default:	all

all:	main

main:	main.c
	$(CC)	$(CFLAGS)	-o	main.out	main.c	-lm

shoppingcart:	projects/shoppingcart.c
	$(CC)	$(CFLAGS)	-o	projects/shoppingcart.out	projects/shoppingcart.c

clean veryclean:
	$(RM)	Hello