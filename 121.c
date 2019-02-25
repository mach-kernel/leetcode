int maxProfit(int* prices, int pricesSize) {
    int bestProfit = 0;
    
    for (int i = 0; i < pricesSize; ++i) {
        int initialLoss = prices[i] * -1;
        
        for (int j = i + 1; j < pricesSize; ++j) {
            if (initialLoss + prices[j] > bestProfit) {
                bestProfit = initialLoss + prices[j];
            }
        }
    }
    return bestProfit;
}
