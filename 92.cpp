/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* reverseBetween(ListNode* head, int m, int n) {
        ListNode* bumper = head;
        
        ListNode* before_start = NULL;
        ListNode* start = NULL;
        ListNode* previous = NULL;
        
        if (n-m == 0) { return head; }
        
        int count = 1;
        while(bumper) {
            ListNode* cached_next = bumper->next;
            if (count == m - 1) { before_start = bumper; }
    
            if (count >= m) {
                if (count == m) { start = bumper; }
                bumper->next = previous;
               
                if (count == n) {
                    start->next = cached_next;
                    if (before_start == NULL) { head = bumper; } 
                    else { before_start->next = bumper; }
                    break;
                }
            }
            
            previous = bumper;
            bumper = cached_next;
            ++count;
        }
        

        return head;
    }
};
