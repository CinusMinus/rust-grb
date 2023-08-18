Algorithm used to solve continuous models or the initial root relaxation of a MIP model. Options are:

-1=automatic,

0=primal simplex,

1=dual simplex,

2=barrier,

3=concurrent,

4=deterministic concurrent, and

5=deterministic concurrent simplex.

Available settings and default behaviour depend on the model type or the type of the initial root relaxation. In the
current release, the default Automatic (-1) setting will typically choose non-deterministic concurrent (Method=3) for an
LP, barrier (Method=2) for a QP or QCP, and dual (Method=1) for the MIP root relaxation. If the size of the MIP root
relaxation is large, then it will often select deterministic concurrent (Method=4) or deterministic concurrent simplex
(Method=5).

Concurrent methods aren't available for QP and QCP. Only the simplex and barrier algorithms are available for continuous
QP models. If you select barrier (Method=2) to solve the root of an MIQP model, then you need to also select barrier for
the node relaxations (i.e. set NodeMethod=2). Only barrier is available for continuous QCP models. However if you choose
LP relaxations for solving MIQCP, you can also select the simplex algorithms (Method=0 or 1).

Concurrent optimizers run multiple solvers on multiple threads simultaneously, and choose the one that finishes first.
Method=3 and Method=4 will run dual simplex, barrier, and sometimes primal simplex (depending on the number of available
threads). Method=5 will run both primal and dual simplex. The deterministic options (Method=4 and Method=5) give the
exact same result each time, while Method=3 is often faster but can produce different optimal bases when run multiple
times.

The default setting is rarely significantly slower than the best possible setting, so you generally won't see a big gain
from changing this parameter. There are classes of models where one particular algorithm is consistently fastest,
though, so you may want to experiment with different options when confronted with a particularly difficult model.

Note that if memory is tight on an LP model, you should consider using the dual simplex method (Method=1). The
concurrent optimizer, which is typically chosen when using the default setting, consumes a lot more memory than dual
simplex alone.

In multiobjective LP optimization:

The first objective is solved using LP defaults. It can be set by the user using the `Method` parameter.

Subsequent objectives are solved by default using primal simplex to allow for warm starting. The method used here can be
controlled using MultiObjMethod.