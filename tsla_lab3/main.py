import enum
from itertools import chain, combinations
import networkx as nx
import matplotlib.pyplot as plt

class SymbolType(enum.Enum):
    NonTerminal = 'NoтTerminal'
    Terminal = 'Terminal'


class Symbol:
    symbolType = SymbolType.Terminal
    value = ''

    def __init__(self, value, symbolType):
        self.value = value
        self.symbolType = symbolType

    def __str__(self):
        # return '{' + self.symbolType.value + '; ' + self.value + '}'
        return ' ' + self.value + ' '


def all_subsets(arr):
    s = list(arr)
    return list(chain.from_iterable(combinations(s, r) for r in range(1, len(s) + 1)))


def filterList(l):
    return len(l[1]) > 0


class CA:
    q = []
    t = []
    f = {}
    h = []
    z = []
    start = ''

    def fromGrammar(self, startSymbol, nonTerminals, terminals, rules):
        self.start = startSymbol
        self.q = []
        self.t = []
        self.f = {}
        self.h = []
        self.z = []
        # step 1:
        self.q.append('N')
        # self.t.append('N')
        # step 2:
        # self.h.append(Symbol(startSymbol, SymbolType.Terminal))
        self.h.append(startSymbol)
        for nonTerminal in nonTerminals:
            self.q.append(nonTerminal)
        for terminal in terminals:
            self.t.append(terminal)
        # step 3
        # step 4
        # step 5
        for left in rules:
            for rule in rules[left]:
                if left == startSymbol and rule[0].value == 'e':
                    self.z.append(startSymbol)
                if len(rule) > 1:
                    nonTerminal = rule[0]
                    terminal = rule[1]
                    if rule[0].symbolType == SymbolType.Terminal:
                        terminal, nonTerminal = nonTerminal, terminal
                    self.f[(left, terminal.value)] = nonTerminal.value
                    self.z.append(nonTerminal.value)
                else:
                    self.f[(left, rule[0].value)] = 'N'
                    # self.z.append(rule[0].value)
                    self.z.append('N')
        self.z = list(set(self.z))
        print('set of states: ', self.q)
        print('input symbols: ', self.t)
        print('change funtion: ', self.f)
        print('set of start symbols: ', self.h)
        print('set of end symbols: ', self.z)
        print('from grammar done')
        # step 6
        # G = nx.Graph()
        # q = list(map(lambda s: ' '.join(s), self.q))
        # print('q: ', self.q)
        # G.add_nodes_from(self.q)
        # for (key, v) in self.f:
            # print(key, v, self.f[(key, v)])
            # G.add_edge(key, self.f[(key, v)])
        # nx.draw_networkx(G)
        # plt.show()
        self.convertToDsa()

    def convertToDsa(self):
        q = all_subsets(self.q)
        q.sort(key=len)
        print('set of states: ', q)
        z = []
        for state in q:
            for s_old in self.z:
                if s_old in state:
                    z.append(state)
                    break
        print('set of end states: ', z)
        f = {}
        print('-------------')
        for (state, terminal) in self.f:
            print(state, terminal)
            f[(tuple(state), terminal)] = [self.f[(state, terminal)]]
        print(f)
        print('start loop')
        for t in self.t:
            print(t)
            for state in q:
                resState = []
                for s in state:
                    if (tuple(s), t) in f:
                        resState.extend(f[(tuple(s), t)])
                f[(tuple(state), t)] = resState
        self.f = list(filter(filterList, f.items()))
        print('change function: ', self.f)
        self.q = q
        self.z = z
        print('convert to dsa done')

    def __str__(self):
        return 'to_str()'

    def show(self):
        G = nx.Graph()
        q = list(map(lambda s: ' '.join(s), self.q))
        print('q: ', q)
        G.add_nodes_from(q)
        for (key, value) in self.f:
            print(key, value)
            G.add_edge(' '.join(list(key[0])),  ' '.join(value))
        nx.draw_networkx(G)
        plt.show()


def parsePart(combined, terminal, nonTermial):
    for terminal in terminals:
        if combined.startswith(terminal):
            nonTermial = combined[len(terminal):]
            return [Symbol(terminal, SymbolType.Terminal), Symbol(nonTermial, SymbolType.NonTerminal)]
    for nonTermial in nonTerminals:
        if combined.startswith(nonTermial):
            terminal = combined[len(nonTermial):]
            return [Symbol(nonTermial, SymbolType.NonTerminal), Symbol(terminal, SymbolType.Terminal)]
    raise RuntimeError('bad rule part: {p}'.format(p = combined))

def parseRigthpart(str, terminals, nonTerminals):
    opts = str.split('|')
    # print(opts)
    res = []
    for opt in opts:
        symbols = []
        pattern = ''
        for c in opt:
            pattern += c
            if pattern in nonTerminals:
                symbols.append(Symbol(pattern, SymbolType.NonTerminal))
                pattern = ''
            elif pattern in terminals:
                symbols.append(Symbol(pattern, SymbolType.Terminal))
                pattern = ''
        res.append(symbols)
    if pattern != '':
        raise RuntimeError(pattern + ' is not an alphabet symbol')
    return res

def printRule(rule):
    res = []
    for symbols in rule:
        res.append('[')
        for symbol in symbols:
            res.append(str(symbol))
        res.append(']')
        res.append('|')
    del res[-1]
    return ''.join(res)

def checkIfRegular(rules):
    if len(rules) < 1:
        raise RuntimeError('Grammar should contain at least one rule')
    for left in rules:
        # print(left)
        for rule in rules[left]:
            # print(rule)
            if len(rule) > 2:
                raise RuntimeError('Grammer is not regular')
    print('Grammar is regular')


input = open("lab_input.txt")
lines = input.readlines()
# print(lines)
if len(lines) < 4:
    raise 'Input file has a bad structure'
nonTerminals = lines[0].strip().split(',')
terminals = lines[1].strip().split(',')
terminals.append('e')
rawRules = list(map(lambda x: x.split('->'), lines[2].strip().split(';')))
rules = {}
for rule in rawRules:
    rules[rule[0]] = parseRigthpart(rule[1], terminals, nonTerminals)

startSymbol = lines[3].strip()

print('nonTerminals: ', nonTerminals)
print('terminals: ', terminals)
# print('rules:' + str(rules))
print('rules:')
for left in rules:
    print(left + ' -> ' + printRule(rules[left]))

print('startSymbol: ', startSymbol)

checkIfRegular(rules)

ca = CA()
ca.fromGrammar(startSymbol, nonTerminals, terminals, rules)
#c print(ca)
ca.show()
