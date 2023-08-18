Limits the number of feasible MIP solutions found. Optimization returns with a SOLUTION_LIMIT status once the limit has
been reached (see the Status Code section for further details). To find a feasible solution quickly, Gurobi executes
additional feasible point heuristics when the solution limit is set to exactly 1.

Note: Only affects mixed integer programming (MIP) models