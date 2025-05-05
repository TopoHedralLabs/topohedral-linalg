import numpy as np
import scipy as sci

def test_eig():

    m1 = np.array([
            [1.0, 5.0, 0.0],
            [2.0, 4.0, -1.0],
            [0.0, 2.0, 3.0]
    ])

    eig_out = sci.linalg.eig(a = m1, left = True, right = True)
    print(eig_out[0])
    print(eig_out[1])
    print(eig_out[2])

def test_non_diagonal_dominant():

    m1 = np.array([[1.0, 2000.0, 3000.0], 
                  [5000, 10, -8900], 
                  [-10000, 9008, 0]])

    p, l, u= sci.linalg.lu(m1)
    print(p)
    print(l)
    print(u)


def test_diagonal_dominant():
    m1 = np.array([[100000.0, 10,      56,      10], 
                   [-69,      1.56e6,  3,       -9], 
                   [0,        0,       -5.6e-5, -700], 
                   [890, 0, -7899, 8e5]])

    p, l, u= sci.linalg.lu(m1)
    print(p)
    print(l)
    print(u)


def test_symmetric_eig():

    m1 = np.array([[1.0, 2.0, 3.0], 
                   [2.0, 4.0, 5.0], 
                   [3.0, 5.0, 6.0]])

    eig_out = sci.linalg.eigh(a = m1, lower = True, type=2)
    print(np.array2string(eig_out[0], precision=14, suppress_small=False, formatter={'float': lambda x: f'{x:.14e}'}))
    np.set_printoptions(precision=14, suppress=False, formatter={'float': lambda x: f'{x:.14e}'})
    print(eig_out[1])
    np.set_printoptions(precision=8, suppress=False, formatter=None)



def main():

    # test_non_diagonal_dominant()
    # test_diagonal_dominant()
    # test_eig()
    test_symmetric_eig()



if __name__ == "__main__":
    main()