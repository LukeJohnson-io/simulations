import simulations as sim
import numpy as np
import matplotlib.pyplot as plt
from scipy.fft import fft, fftfreq    


def string():
    initial = [
        -0.1, 0.1, 0.0,
        0.0, 0.0, 0.0,
        ]
    time_final = 2
    step_size = 0.001
    mass = 0.1
    stiffness = 10.0

    steps = int(time_final/step_size)
    data = np.array(sim.string(initial, steps, step_size, mass, stiffness))
    t = np.linspace(0, time_final, steps)
    fig, ax = plt.subplots()

    ax.plot(t, data[:,0])
    ax.plot(t, data[:,3])

    

def main():
    initial = [
        0.0, 0.0, 0.0, -0.1, 0.1, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
    ]
    time_final = 10
    step_size = 0.0001
    mass = 0.1
    stiffness = 100.0

    steps = int(time_final/step_size)
    data = np.array(sim.string(initial, steps, step_size, mass, stiffness))
    t = np.linspace(0, time_final, steps)
    fig, ax = plt.subplots()

    yf = fft(data[:,3])
    xf = fftfreq(steps, step_size)[:steps//2]

    # ax.plot(xf, 2.0/steps * np.abs(yf[0:steps//2]))
    ax.plot(t, data[:,3])
    plt.show()


if __name__ == '__main__':
    main()
