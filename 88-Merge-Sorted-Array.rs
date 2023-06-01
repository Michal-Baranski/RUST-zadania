pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p1 = m - 1; // Inicjalizacja wskaźnika p1 na ostatni element nums1
    let mut p2 = n - 1; // Inicjalizacja wskaźnika p2 na ostatni element nums2
    let mut p = m + n - 1; // Inicjalizacja wskaźnika p na ostatni element wynikowego nums1

    // Pętla trwa, dopóki wskaźniki p1 i p2 są większe lub równe 0
    while p1 >= 0 && p2 >= 0 {
        // Porównaj elementy nums1[p1] i nums2[p2]
        // i umieść większy z nich na odpowiedniej pozycji w nums1[p]
        if nums1[p1 as usize] > nums2[p2 as usize] {
            nums1[p as usize] = nums1[p1 as usize];
            p1 -= 1;
        } else {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
        }
        p -= 1; // Przesuń wskaźnik p w lewo
    }

    // Jeśli w nums2 są pozostałe elementy, skopiuj je do nums1
    while p2 >= 0 {
        nums1[p as usize] = nums2[p2 as usize];
        p2 -= 1;
        p -= 1;
    }
}


  /*
  You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
  */
