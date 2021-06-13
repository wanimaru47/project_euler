#include <bits/stdc++.h>
using namespace std;

int main() {
    int N(20), M(20);

    vector<vector<int>> v(N, vector<int>(M));
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            cin >> v[i][j];
        }
    }

    for (auto i : v) {
        for (auto j : i) {
            cout << j << " ";
        }
        cout << endl;
    }

    int ans = 0;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int r(0); if (j + 3 < M) {r = v[i][j] * v[i][j+1] * v[i][j+2] * v[i][j+3];}
            int d(0); if (i + 3 < N) {d = v[i][j] * v[i+1][j] * v[i+2][j] * v[i+3][j];}
            int e(0); if (i + 3 < N && j + 3 < M) {e = v[i][j] * v[i+1][j+1] * v[i+2][j+2] * v[i+3][j+3];}
            int k(0); if (i + 3 < N && j - 3 >= 0) {k = v[i][j] * v[i+1][j-1] * v[i+2][j-2] * v[i+3][j-3];}

            ans = max(ans, max(max(r, d), max(e, k)));
        }
    }

    cout << ans << endl;

    return 0;
}
