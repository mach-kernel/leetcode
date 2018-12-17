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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* reversed = reverse(head);
        if (n == 1) return reverse(reversed->next);
        
        ListNode* last = NULL;
        ListNode* dummy = reversed;
        
        for(int i = 1; dummy; ++i) {
            if (i == n  && last) last->next = dummy->next;
            last = dummy;
            dummy = dummy->next;
        }
        
        return reverse(reversed);
    }
private:
    ListNode* reverse(ListNode* head) {
        ListNode* last = NULL;
        ListNode* dummy = head;
        
        while (dummy) {
            ListNode* next = dummy->next;
            dummy->next = last;
            last = dummy;
            dummy = next;
        }
        
        return last;
    }
};
