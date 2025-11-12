# NP Complete Project

## Learning Objective

- implement a brute-force approach to solve a known NP-complete problem and
  analyze its runtime complexity
- present/discuss an NP-completeness proof to a group of colleagues and justify
  that an approximation method is appropriate
- create an approximation method for a known NP-complete problem and present its
  strengths and weaknesses.
- implement a reduction to another NP complete problem and compare the
  performance
- improve presentation skills (both speaking and slide creation) by giving and
  receiving peer feedback

## Introduction

In this project you will explore a known NP-complete problem. The sections below
detail each of the project components. Individual assignments will be created
for all submission pieces (or they will be submitted in Github and an assignment
will be created in Canvas to represent your grade).

## Problem and Group Selection

This is a group project that must have 3 members. It may be necessary to work in
a group of 2, and if so, the instructor will work out the details of what
portions of the project can be reduced in scope. Your group will select and rank
three NP-complete problems that you would like to work on for this project from
the list below. You should do some research before selecting one of these
problems. Ideally, we will get a nice distribution of projects across these
problems.

- Max 3-Sat
- Max clique
- Traveling Salesman Problem (TSP) with a complete graph
- Longest Path
- Min Vertex Cover
- Min Graph Coloring

I have created example input and output files for each problem. After I receive
and review all the group preferences, I will assign each group a single
NP-complete problem to work on.

## Role Selection and Tasks

Each of the tasks for the project are outlined below. One person in the group
will be responsible for Parts **A** and **B**, another will perform Parts **C**
and **D** and another will perform Parts **E** and **F**.

### Part A - Exact Solution Code

Develop Python code that provides the optimal solution to the problem you are
studying. You must develop test cases of varying sizes and illustrate how the
runtime (wall clock) of your program varies for different size input. Your test
cases must include at least one instance that causes your program to run for
more than 20 minutes.

Make sure you cite in the comments of your program ALL external sources used in
the development of your code.

#### Submission

Create a folder named **exact_solution** with the GIT repository. Place your
code in this folder including a **README.txt** file that provides instructions
and the **exact commands** to run your code. Within the exact_solution folder,
create a **test_cases** subfolder that contains all test cases. Create a shell
script named **run_test_cases.sh** that executes your program on all test cases.
Clearly label (with comments) which test case causes your program to run for
more than 20 minutes. An example set of shell scripts for performing this work
is provided (thanks to Martin Nester for developing these very nice scripts).

Include in the write up:

- why the problem is important (some applications that use this problem)

### Part B - Problem Presentation

Perform a 5 minute presentation on your group’s work for Part A. This
presentation must include:

- a description of the problem being solved. Show the difference between the
  decision and optimization version of the problem. An example of inputs and
  outputs is a good idea. (1.5 mins)
- an explanation of the certifier process being polynomial. Including
  pseudo-code here is a good idea but make sure its clear that the certificate
  can be verified in polynomial time.
- a reduction from a known/accepted NP-hard problem (with examples graphically
  shown). It must be clear that the reduction takes place in a polynomial number
  of steps with respect to the problem’s input size.
- an analytical (big OO) runtime analysis of your coded solution and showcase
  the code that is the dominant term (the code that drives the highest order
  term).
- a wallclock (empirical) runtime analysis of your code on your test cases. This
  analysis must show input size on the X axis and runtime on the Y axis.

#### Submission

Create a folder named **presentations** with the GIT repository. This
presentation should use slides in either PowerPoint, Keynote, or Google slides
(other slides formats can be requested, but are subject to instructor approval).
The slides (or a file with a link to the google slides) should be placed in the
git repository. Name this presentation **problem_presentation** with whatever
file extension is appropriate.

### Part C - Develop and Approximation Solution and Code It

Develop Python code that provides a reasonable approximation of the solution.
Your approximation **must run in polynomial time** and process large problems
(n>1000) quickly. Some popular strategies are:

- Make greedy local choices, breaking ties randomly
- utilize randomness coupled with an anytime algorithm

Your program must execute without arguments (so that it works on gradescope),
but you can incorporate optional command line arguments to augment the function
of your program (for example, output wall clock timings or override the runtime
for an anytime approach).

#### Submission

Create a folder named **approx_solution** with the GIT repository. Place your
code in this folder and create a **README** file that provides instructions on
how to run your program. Within the approx_solution folder, create a
**test_cases** subfolder that contains all test cases. You must create a file
named **run_test_cases.sh** that successfully runs your program on several test
cases you created. One of these test cases **must** be one where your
approximation solution does not achieve the optimal answer, place this test case
in **run_test_cases.sh** and **run_nonopt_cases.sh**. The
**run_nonopt_cases.sh** file only needs one example. Document notes on your
solution in a file named **approximation_notes.pdf** and at a minimum these
notes must addresses the items listed in the rubric below.

### Part D - Approximation Presentation

Perform a **5 minute** presentation on your group’s work for Part C.

This presentation must include:

- pseudocode for the approximation and a discussion of the strategy used
  (greedy, random, etc.)
- analytical run time analysis (the worse case OO time) of your approximation
  algorithm
- plots that illustrate the run-time (wall clock) performance of your exact
  solution versus the approximation solution on your test cases
- plots that compare the result/solution of your exact solution versus the
  approximation on your test cases. This is where you show the value achieved by
  both and see how well your approximation did versus the optimal solution.

For the data that is used to create run times on your test cases, you should
provide these in a script named **compute_approx_wallclock.sh** (this should
reside in the test_cases folder). Running this script should run your test cases
that you used to gather your data for the plots. Recall that UNIX has a **time**
command that may proof useful in collecting this information.

#### Submission

Within the folder presentation add a presentation/slides file constructed in
PowerPoint, Keynote, or Google slides (other slides formats can be requested,
but are subject to instructor approval). The slides (or a file with a link to
the google slides) should be placed in the git repository. Name this
presentation **approximation_presentation** with whatever file extension is
appropriate.

### Part E - Augment the Study

For this portion of the project, you will need to select **one** of the optional
components for each of the problems. These will be available in the next day,
but this person’s role will be to help augment the approximation solution.

### Part F - Present

This will be to present on your work done in Part E.

## Rubrics

### Common rubric (13 points)

| **Item**                                        | **Points** |
| ----------------------------------------------- | ---------- |
| Team Formation                                  | 3          |
| Student Participation during in-class work days | 6          |
| Peer review submission                          | 4          |

### Exact solution rubrics (18 points).

| **Item**                                                          | **Points** |
| ----------------------------------------------------------------- | ---------- |
| Gradescope tests                                                  | 8          |
| Small test cases are documented and explained to show correctness | 4          |
| Large test case (more than 20 mins to run) is discussed           | 3          |
| run_test_cases.sh script works and executes all test cases        | 3          |

### Exact solution presentation rubrics (18 or 19 points).

| **Item**                                                                                                         | **Points** |
| ---------------------------------------------------------------------------------------------------------------- | ---------- |
| Problem Description (what, why, applicaitons). Illustrate                                                        | 8          |
| Show proof of NP-Completeness (decision, NP certificate, and reduction). Include a small sample of the reduction | 4          |
| Longest Path Only (show why taking negative weights on all edges and using Bellman-Ford does not work)           | 1          |
| Sketch Exact Solution                                                                                            | 3          |
| Discuss test case generation                                                                                     | 3          |

### Approximation code (18 points).

Discussion points should be in approximation_notes.pdf within the
approx_solution folder in the github repo.

| **Item**                                                                                             | **Points** |
| ---------------------------------------------------------------------------------------------------- | ---------- |
| Explain strategy (greedy, anytime, etc.).                                                            | 3          |
| Discuss the analytical runtime analysis of your approximation. Recall it must run in polynomial time | 6          |
| Program performs “reasonably well” on test cases on Gradescope                                       | 6          |
| run_test_cases.sh showscases at least 3 test cases (1 with nonoptimal results)                       | 3          |

### Approximation Presentation (18 points).

| **Item**                                                                                                                                                                   | **Points** |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| Explain strategy (greedy, anytime, etc.) and how the code works                                                                                                            | 3          |
| Discuss problem instance where approximation does not achieve the optimal value. Use illustrations.                                                                        | 3          |
| Provide wallclock runtime results comparing the exact and approx solutions on one plot.                                                                                    | 6          |
| Discuss a lower bound for your solution and show the delta to this lower bound (from the value you computed) for large problem instances you created for _run_examples.sh_ | 6          |
