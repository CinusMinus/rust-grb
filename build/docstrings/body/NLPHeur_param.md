The NLP heuristic uses a non-linear barrier solver to find feasible solutions to non-convex quadratic models. It can
often find solutions much more quickly than the alternative, but in some cases it can consume significant runtime
without producing a solution. By default, the heuristic is enabled (1). Use 0 to disable the heuristic.

Note: Only affects models with nonconvex quadratic expressions in the objective or constraints