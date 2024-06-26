# 1. Radix Scrypto Simple Bank account example

Running the example is much the same as the running the original Hello example (https://github.com/radixdlt/official-examples/tree/main/step-by-step/02-hello-token-explained)

0.  First, reset the simulator and create a new account.
    ```sh
    resim reset
    resim new-account
    resim show <ACCOUNT_ADDRESS>
    ```

1.  Then, publish the package and save the package address.
    ```sh
    resim publish .
    ```
2.  Use the package address to instantiate the component and save the component address

    ```sh
    resim call-function <PACKAGE_ADDRESS> BankAccount instantiate
    ```

3.  This is a good opportunity to check the state of the component. Should display balance Euro

    ```sh
    resim show <COMPONENT_ADDRESS>
    resim call-method <COMPONENT_ADDRESS> get_balance
    ```

4.  Now we can transfer some balances
    ```sh
    resim call-method <COMPONENT_ADDRESS> withdraw(<Amount>)  //withdraw Euro
    resim call-method <COMPONENT_ADDRESS> deposit <Resource>:<Amount>   //deposit XRD or EURO
    ```

