#include <iostream>

using namespace std;

int hello(int n){
	cout<<"Hello #"<<n<<endl;

	return(n+1);
}

int main(){
	cout<<hello(5)<<endl;

	return(0);
}
