#include <stdio.h>

void test()
{
	int sum=0;
	for(int i=0;i<10;i++)
		for(int j=0;j<100;j++)
			sum+=i+j;
	printf("%d",sum);
}

int main()
{
	test();
	return 0;
}
