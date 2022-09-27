import argparse
from dataclasses import dataclass
from yaml import load, dump
try:
	from yaml import CLoader as Loader
except ImportError:
	from yaml import Loader

#DEFINE
TM = "team_members"
G = "goals"

@dataclass
class Goal:
	desc: str
	prs: list[int]
	status: str

	def check(self):
		pr_check = [ isinstance(pr, int) for pr in self.prs ]
		if pr_check.count(True) < len(self.prs):
			raise ValueError(f"Invalid PR in goal: {self}")

@dataclass
class TeamMember:
	pitt_username: str
	score: int
	desc: str

	def check(self):
		if not isinstance(self.score, int) or not (-2 <= self.score <= 2):
			raise ValueError(f"Invalid score of {self.score} for team member {self.pitt_username}")

@dataclass
class Report:
	manager_pitt_username: str
	week: int
	final_commit_id: str
	goals: list[Goal]
	team_members: list[TeamMember]

	def check(self):
		if not isinstance(self.week, int):
			raise ValueError(f"Invalid week: {self.week}")

		if not (len(self.final_commit_id) == 40):
			raise ValueError(f"Invalid commit id (wrong length): {self.final_commit_id}")

		try:
			int(self.final_commit_id, 16)
		except Exception as e:
			raise ValueError(f"Invalid commit id (invalid hex): {self.final_commit_id}")

def main():
	parser = argparse.ArgumentParser(description="sanity check a management week report YAML file")
	parser.add_argument("fname", help="file name to check")
	args = parser.parse_args()

	print("Checking...")

	try:
		with open(args.fname) as inf:
			report = load(inf, Loader=Loader)
			check_report(report)
	except Exception as e:
		print("ERROR!\n")
		raise RuntimeError("Could not open/parse YAML report") from e
	else:
		print("OK!")

def check_report(report):
	report[TM] = [ TeamMember(**tm) for tm in report[TM] ]
	report[G] = [ Goal(**g) for g in report[G] ]
	r = Report(**report)
	r.check()
	for g in r.goals:
		g.check()
	for t in r.team_members:
		t.check()

if __name__ == "__main__":
	main()
