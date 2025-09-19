#include <stdio.h>
#include <stdlib.h>
int main() {
	int *p1 = malloc(100);
	int *p2 = p1;
	free(p1);
	free(p2);
}
