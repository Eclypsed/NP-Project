"""
name:  Alston Knight

Honor Code and Acknowledgments:

        This work complies with the JMU Honor Code.

        Wrapper for running C binary (May be needed on gradescope or for submission)
"""


import sys
import subprocess


def main():

    data = sys.stdin.read()

    proc = subprocess.Popen(["./bin/cs412_longestpath_approx"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    out, err = proc.communicate(data)

    if out:
        print(out, end="")

    if err:
        print(err, file=sys.stderr, end="")


    sys.exit(proc.returncode)


if __name__ == "__main__":
    main()
