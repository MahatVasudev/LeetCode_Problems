#include <iostream>
#include <queue>
#include <vector>

using namespace std;

void addEdge(vector<vector<int>> addj, int u, int v) {
  addj[u].push_back(v);
  addj[v].push_back(u);
}

int main() {
  int V = 5;

  vector<vector<int>> adj(V);

  addEdge(adj, 0, 1);
  for (int i; i < 5; i++) {
    for (int j; j < 5; j++) {
      cout << adj[i][j];
    }
  }
}
