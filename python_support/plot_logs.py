# This is a sample Python script.
import sys
from pathlib import Path

from matplotlib import pyplot as plt

# Creates plots from the last START_PLOT and STOP_PLOT found in the provided logs file
def parse_log_line(line: str):
    s = line.split('| ')
    if len(s) > 1:
        return s[1].replace('\n', '')
    return ''


def extract_values(input_str: str):
    m = input_str.replace('(', '').replace(')', '').replace(' ', '')
    x, y, z = m.split(',')
    return x, y, z


def extract_line_names(names_and_values: str) -> list[str]:
    return [p.split('=')[0] for p in names_and_values.split(',')]


def extract_points(names_and_values: str) -> list[float]:
    return [float(p.split('=')[1]) for p in names_and_values.split(',')]


class GraphHolder:
    def __init__(self, name: str, line_names: list[str]):
        self._name = name
        self._line_names = line_names
        self._points: list[list[float]] = [[] for i in range(len(line_names))]

    def add_points(self, points: list):
        for i, point in enumerate(points):
            self._points[i].append(point)
        # self.points.append(points)

    def create_graph(self):
        for i, line_points in enumerate(self._points):
            plt.plot(line_points)
        plt.title(self._name)
        plt.legend(self._line_names)
        plt.show()


def create_graph(cur_graph):
    graphs = {}
    for measurement in cur_graph:
        m = measurement.split('_')
        if m[0] != 'P':
            continue
        graph_name = m[1]
        if graph_name not in graphs:
            graphs[graph_name] = GraphHolder(graph_name, extract_line_names(m[2]))
        graphs[graph_name].add_points(extract_points(m[2]))
    for graph in graphs.values():
        graph.create_graph()


def main():
    # my dolphin-emu log file is located at:
    # /home/knaap/.local/share/dolphin-emu/Logs/dolphin.log
    if not len(sys.argv) == 2:
        print("run using: plot_logs.py <path_to_log_file")
        exit()
    log_file_path_str = sys.argv[1]
    log_file = Path(log_file_path_str)

    start_plot = 0
    stop_plot = 0
    with open(log_file, 'r') as f:
        file_content = f.readlines()

    for i, line in enumerate(file_content):
        parsed = parse_log_line(line)
        if parsed == 'START_PLOTS':
            start_plot = i
        elif parsed == 'STOP_PLOTS':
            stop_plot = i
    cur_graph = [parse_log_line(file_content[line_i]) for line_i in range(start_plot+1, stop_plot)]
    create_graph(cur_graph)


if __name__ == '__main__':
    main()
