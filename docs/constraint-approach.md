---
title: Bus Charging Schedule Simulated Annealing
---

This document outlines the simulated annealing approach to the bus charging scheduling problem.

# Simulated Annealing Requirements
* Initial Temperature
* Cooling schedule (temperature function)
* Generation mechanism
* Repetition schedule ($\pi_k$)

# Optimization Problem

## Objective Function
The objective function(s) should take into consideration

* Time on the charger (consumption charge)
* Which charger (slow/fast/index)
* Power usage (Peak times/P15)
* Time (Peak times/P15)
* Temperature (Encourage exploration at the beginning and discourage near the end) TODO: Find reference on this

Let $J$ represent the objective function. From what can be gathered from the considerations above it can be said that the objective function has four main considerations

* Assignment
* Power
* Time
* Temperature TODO: Find reference

which would be of the form $J = (AC + PC)$. $T$ is the temperature, $AC$ is the assignment cost, and $PC$ is the power usage cost. The assignment cost can be broken down into two components:

$$
AC = \sum_{i=1}^I \sum_{q=1}^Q c_i w_{iq} \epsilon_q
$$

Where $w_{iq}$ is the assignment of visit $i$ to charger $q$, $c_i$ is the charge duration for visit $i$, and $\epsilon_q$ is the cost of usage for charger $q$. The first portion being the cost of assignment and the second being the cost of use. The consumption cost is represented as

$$
PC = \sum_{i=1}^I \sum_{q=1}^Q w_{iq} r_q c_i
$$

where $r_q$ is the wattage of the charger (KW). Peak 15 should also be taken into consideration. P15 can be written as:

$$
\rho_{15}(t) = 1/15 \int_{t-15}^{t} \rho(\tau) d\tau
$$

because worst case must be assumed to always ensure enough power is supplied

$$
\rho_{max}(t) = \text{max}_{\tau\in [0,t]}\rho_{15}(\tau)
$$

The demand charge is then determined by

$$
\rho_{d}(t) = \text{max}(\rho_{fix},\rho_{max}(t))s_r
$$

where $s_r$ is the demand rate. From this we can write:

$$
PC = \sum_{i=1}^I \sum_{q=1}^Q w_{iq} r_q c_i + \rho_{T}
$$


## Constraints
Now that a method of calculating the fitness of a schedule has been established, a method for determining if the schedule is feasible must be determined. Feasible schedule require

* No overlap in time
* No overlap in space
* Bus receives enough charge
* Leaves on time

$$
\begin{array}{ll}
	p_i \geq u_j \;||\; p_j \geq u_i                                   & \text{Valid time}                                                        \\
	q_i \;!= q_j                                                         & \text{Valid position}                                                    \\
	p_i + u_i = c_i                                                    & \text{Valid depart time (TODO: redundant?)}                              \\
	a_i \leq u_i \leq (T-p_i)                                          & \text{Arrival time < initial charge time < maximum initial charge time}  \\
	c_i \leq \tau_i                                                    & \text{Detatch time should be less than or equal to departure time}       \\
	\eta_{\gamma_i} = \eta_i + \sum_{q=1}^Q p_i w_{iq} r_q + \lambda_i & \text{Charge constraint (initial and final charges can also be applied)} \\
	\eta_{\gamma_i} - \lambda_i \geq 0                                 & \text{Sufficient charge is supplied to the bus}
\end{array}
$$

## Cooling Equation (Experimental)
There are three basic types of cooling equations as shown in Fig \ref{fig:cool}.

![Cooling equations \label{fig:cool}](img/cool-func.png)

# Generation Mechanism(s)
For the case of the bus generation, three generation mechanism shall be used.

* Route generation (Fig \ref{fig:route})

![Cooling equations \label{fig:route}](img/route_generation.png)
	
* Schedule generation (Fig \ref{fig:schedule})

![Cooling equations \label{fig:schedule}](img/charge_solution.png)

* Tweak schedule (Fig \ref{fig:tweak})

![Cooling equations \label{fig:tweak}](img/charge_tweak.png)
