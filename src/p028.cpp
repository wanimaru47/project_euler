#include <bits/stdc++.h>
using namespace std;

int main() {
    int N = 1001;
    int sum  = 1;

    int d = 1;
    for (int i = 2; i <= N; i += 2) {
        for (int j = 0; j < 4; j++) {
            sum += (d += i);
        }
    }

    cout << sum << endl;

    return 0;
}
