impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut result = Vec::new();
        let n = nums.len();

        //obliczamy maksymalną wartość maski, która ma zbiory o rozmiarze n.
        let max_mask = 1 << n;

        //iterujemy po maskach (wszystkich)
        for mask in 0..max_mask
        {
            let mut subset = Vec::new();

            //dodajemy do zbioru liczby, których indeksy odpowiadają ustawionym bitom w masce.
          for i in 0..n
          {
              if mask & (1 << i) != 0
              {
                  subset.push(nums[i]);
              }
          }

          //dodajemy nowy zbór do wyniku
          result.push(subset);
        }
        result

    }
}

/*
Given an integer array nums of unique elements, return all possible 
subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.

Example 1:
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

Example 2:
Input: nums = [0]
Output: [[],[0]]
*/
