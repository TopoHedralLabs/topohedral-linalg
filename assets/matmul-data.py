
import numpy as np
import json


def main():

    out = dict()
    for N in range(1, 11):
        out[N] = dict()
        for k in range(1, N):

            m1 = np.asfortranarray(np.random.uniform(low=0, high=100, size=(N, k)))
            m2 = np.asfortranarray(np.random.uniform(low=0, high=100, size=(k, N)))
            m3 = np.matmul(m1, m2)

            out[N][k] = dict()
            out[N][k]['m1'] =  {
                "data": m1.ravel().tolist(), 
                "nrows": N, 
                "ncols": k,
            }
            out [N][k]['m2'] =  {
                "data": m2.ravel().tolist(),
                "nrows": k,
                "ncols": N,
            }
            out[N][k]['m3'] =  {
                "data": m3.ravel().tolist(),
                "nrows": k,
                "ncols": N,
            }

    with open('matmul-data.json', 'w') as f:
        json.dump(out, f, indent=4)
            





    # pass
# def test1():


#     A = np.array([[1,2,3],[4,5,6]])
#     B = np.array([[1,2],[3,4],[5,6]])
#     C = np.matmul(A,B)
#     print(C)


# def test2():

#     A = np.array([[1,2,3],[4,5,6]])
#     b = np.array([1,2,3])
#     c = np.matmul(A,b)
#     print(c)



# def main():

#     test1()
#     test2()

if __name__ == '__main__':
    main()