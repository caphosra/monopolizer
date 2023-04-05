from subprocess import Popen, PIPE
import requests
import time

class GameProxy:
    def __init__(self) -> None:
        self.proc = Popen(["cargo", "run", "--bin=mplz"], stdin=PIPE, stdout=PIPE)
        time.sleep(3)

    def load(self, file_name: str):
        print(requests.post("http://127.0.0.1:5391/load", file_name).text)

    def analyze(self, result_file: str, iteration: int, simulation_turn: int):
        json_data = {
            "file_name": result_file,
            "iteration": iteration,
            "simulation_turn": simulation_turn
        }
        print(requests.post("http://127.0.0.1:5391/analyze", json=json_data).text)

    def kill(self):
        self.proc.kill()
