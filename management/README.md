# CS1666 Managment Weeks

Each team will need to construct a Markdown file to note who will manage the
team for each management week. If your team does not have enough members to
staff all weeks, you can list "NONE" for any unused management week. Each
student must be a manager for a single week.

A template form for end of managment week reports can be found in
`TEAMNAME_wkXX__mgmt_report.yaml`. Before sending a copy to me, rename it so as
to replace `TEAMNAME` with your team name, and the `XX` with your week number.
Each team member's Effort rating should be on the following scale:

* -2: No effort or next-to no effort
* -1: Insufficient effort
* 0: Sufficient effort
* 1: Extra effort
* 2: Overwhelming effort

I will be happy if everyone earns a 0 for each week all term, and will be
unhappy if there are alot of -2's and +2's earned (both indicate an imbalance
of effort).

To ensure that I can easily parse all reports and track effort throughout the
term, I have a Python script to verify the YAML file. There are a couple of 
example file provided in `examples/`. You will need PyYAML installed to run
the script
[https://pyyaml.org/wiki/PyYAMLDocumentation](https://pyyaml.org/wiki/PyYAMLDocumentation).
With that, you should be able to simply check as follows:

```
python report_checker.py FILENAME.yaml
```

E.g., using an example file:

```
python report_check.py examples/ExampleTeam_wk01_mgmt_report.yaml
```
