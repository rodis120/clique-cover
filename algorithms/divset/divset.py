#https://www.geeksforgeeks.org/generate-all-partition-of-a-set/
import base64
 
def isclique(table,matrix):
    for i in table:
        for k in table:
            if i!=k and matrix[i][k]==0:
                return False
    return True

def decode(s: str) -> list[list[int]]:
    byte_str = base64.b64decode(s)
    n = int.from_bytes(byte_str[:2], byteorder='little', signed=False)
 
    out = [[0]*n for _ in range(n)]
 
    index = 2
    bit = 1
    for i in range(n):
        for j in range(i, n):
            if((byte_str[index] & bit) != 0):
                out[i][j] = 1
                out[j][i] = 1
 
            bit <<= 1
 
            if(bit == 0x100):
                index += 1
                bit = 1
 
    return out
 
def encode(matrix: list[list[int]]) -> str:
    n = len(matrix)
    byte_str = bytearray()
    byte_str.extend(n.to_bytes(2, byteorder='little', signed=False))
 
    byte = 0
    bit = 1
    for i in range(n):
        for j in range(i, n):
            if matrix[i][j] == 1:
                byte += bit
 
            bit <<= 1
 
            if bit == 0x100:
                byte_str.append(byte)
                byte = 0
                bit = 1
 
    if len(byte_str) < 2 + ((n+1)*n//2 + 7)//8:
        byte_str.append(byte)
 
    return base64.b64encode(byte_str).decode('ascii')

def print_partition(ans):
    """
    Function to print a partition
    """
    t=[]
    for i in ans:
        pom=[]
        #print("{", end=" ")
        for element in i:
            #print(element, end=" ")
            pom.append(element)
        #print("}", end=" ")
        t.append(pom)
    print(t)

def partition_set(set, index, ans,matrix):
    """
    Function to generate all partitions
    """
    
    if index == len(set):
        # If we have considered all elements in the set, print the partition
        print_partition(ans)
 
        return

    
    # For each subset in the partition, add the current element to it and recall
    for i in range(len(ans)):
        

        #tutaj wstawić czy obecny podział indukuje klike
        #print(ans,"##testowe##")

        #patrzenie czy istniejące zbiory tworzą kliki




        '''
        for vertex_list in ans:
            print(vertex_list+[set[index]],isclique(vertex_list+[set[index]],matrix))
            if isclique(vertex_list+[set[index]],matrix):
                break
                #pass'''
        ans[i].append(set[index])
        partition_set(set, index + 1, ans,matrix)
        ans[i].pop()
 
    # Add the current element as a singleton subset and recall
    ans.append([set[index]])
    partition_set(set, index + 1, ans,matrix)
    ans.pop()
 
 
def all_partitions(set,matrix):
    """
    Function to generate all partitions for a given set
    """
    ans = []
    partition_set(set, 0, ans,matrix)
 
 
# Main function
if __name__ == "__main__":
    # The size of the set
    
    matrix = [
        [0,1,1,0],
        [1,0,1,0],
        [1,1,0,1],
        [0,0,1,0]
    ]
    n = len(matrix)
 
    # Initialize the set as {1, 2, ..., n}
    set = list(i for i in range(0,n))
    #set=[0,1,2,3]
    print("All partitions of the set will be:")
    # Generate all partitions of the set
    all_partitions(set,matrix)
    '''
    napis="MgAAACAIAIAAgAIgAACAAAAEAgAAAACQAAQAQAkAAAAAAAYBAAAAAAAwAABAAIAQAAABIAAEAAAAACYECAIAAAAAIAUAABEAEAAAAIAIAAgAAAEBAIACAAAEAAAEAAAFCAAAEAAACAEAAAAAABAAAAAACBAAAKAAAAAAAAgAAAgAAQAAAIBAABAQAAKQAAAAAACCICAACAAgAAAAAAAAABAA"
    matrix=decode(napis)
    n = len(matrix)

    set = list(i for i in range(0,n))
    all_partitions(set,matrix)
    #print(a)
    #print(isclique([0,1,2],matrix))
    '''
