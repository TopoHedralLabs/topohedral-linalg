
import numpy as np


def test1():


    A = np.array([[1,2,3],[4,5,6]])
    B = np.array([[1,2],[3,4],[5,6]])
    C = np.matmul(A,B)
    print(C)


def test2():

    A = np.array([[1,2,3],[4,5,6]])
    b = np.array([1,2,3])
    c = np.matmul(A,b)
    print(c)



def main():

    test1()
    test2()

if __name__ == '__main__':
    main()