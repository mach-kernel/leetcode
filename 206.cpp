/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
 
 // I actually kind of like this!
class Solution {
public:
    ListNode* reverseList(ListNode* head, ListNode* previous = NULL) {
        if (!head) { return previous == NULL ? head : previous; } 
        
        ListNode* cached_next = head->next;
        if (cached_next == previous) { return head; }
        
        head->next = previous;
        return reverseList(cached_next, head);
    }
};
