#include <bits/stdc++.h>
using namespace std;

vector<int> split(int x) {
    vector<int> ret;
    while (x) {
        ret.push_back(x % 10);
        x /= 10;
    }

    return ret;
}

bool check(int x, int y, int z) {
    vector<bool> used(10, false);
    auto sx = split(x);
    for (auto i : sx) {
        if (!used[i]) used[i] = true;
        else return false;
    }
    auto sy = split(y);
    for (auto i : sy) {
        if (!used[i]) used[i] = true;
        else return false;
    }
    auto sz = split(z);
    for (auto i : sz) {
        if (!used[i]) used[i] = true;
        else return false;
    }

    if (used[0]) return false;
    used[0] = true;
    for (auto i : used) if (!i) return false;

    return true;
}

int main() {
    set<int> s;

    for (int i = 0; i < 9999; i++) {
        for (int j = 0; j < 9999; j++) {
            int product = i * j;
            if (check(i, j, product)) s.insert(product);
        }
    }

    int sum = 0;
    for (auto i : s) sum += i;

    cout << sum << endl;

    return 0;
}
