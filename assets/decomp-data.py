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



def main():

    # test_non_diagonal_dominant()
    # test_diagonal_dominant()
    test_eig()

    # for m in range(30):
    #     for n in range(30):
    #         for k in range(30):

    #             m1 = np.random.uniform(low=0, high=100, size=(m, k))
    #             out = sci.linalg.qr
    #             m2 = np.random.uniform(low=0, high=100, size=(k, n))


    # pass


if __name__ == "__main__":
    main()


#  INFO is INTEGER
#           = 0:  successful exit
#           < 0:  if INFO = -i, the i-th argument had an illegal value
#           > 0:  if INFO = i, U(i,i) is exactly zero. The factorization
#                 has been completed, but the factor U is exactly
#                 singular, and division by zero will occur if it is used
#                 to solve a system of equations.