#include <bits/stdc++.h>
using namespace std;

int main() {
    set<int> s;
    for (int a = 2; a <= 100; a++) {
        for (int b = 2; b <= 100; b++) {
            s.insert(floor(pow(a, b)));
        }
    }
    cout << s.size() << endl;

    return 0;
}
