import numpy as np
import scipy as sci


def main():


    for m in range(30):
        for n in range(30):
            for k in range(30):

                m1 = np.random.uniform(low=0, high=100, size=(m, k))
                out = sci.linalg.qr
                m2 = np.random.uniform(low=0, high=100, size=(k, n))


    pass


if __name__ == "__main__":
    main()