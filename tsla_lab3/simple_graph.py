# import numpy as np
# import networkx as nx
import matplotlib.pyplot as plt

# A = np.matrix([[1,1],[2,1]])
# G = nx.from_numpy_matrix(A)
# nx.draw(G)
# plt.show()

# import networkx as nx
# G = nx.Graph()
# G.add_nodes_from(['A', 'B', 'C', 'D'])
# G.add_edge('A', 'B')
# G.add_edge('B', 'D')
# G.add_edge('A', 'C')
# G.add_edge('C', 'D')
# nx.draw(G)
# plt.show()
import networkx as nx

G=nx.Graph()
G.add_node("a")
G.add_nodes_from(["b","c"])

G.add_edge(1,2)
edge = ("d", "e")
G.add_edge(*edge)
edge = ("a", "b")
G.add_edge(*edge)

print("Nodes of graph: ")
print(G.nodes())
print("Edges of graph: ")
print(G.edges())

G.add_edges_from([("a","c"),("c","d"), ("a",1), (1,"d"), ("a",2)])

nx.draw(G)
plt.show() # display
