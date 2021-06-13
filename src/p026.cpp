#include <bits/stdc++.h>
using namespace std;

int main() {
    int ans = 0;
    int res;
    for (int i = 1; i <= 1000; i++) {
        int d = 1;
        map<int,int> p;
        int count = 0;
        while (d) {
            if (p.find(d) != p.end()) {
                if (ans < count - p[d]) {
                    ans = count - p[d];
                    res = i;
                }
                break;
            }
            p[d] = count;
            if (d < i) {
                d *= 10;
            } else {
                count++;
                d %= i;
            }
        }
        cout << i << " " << count << endl;
    }

    cout << res << " " << ans << endl;

    return 0;
}
