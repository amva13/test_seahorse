# test_seahorse
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Operation(Enum):
  ADD = 0
  SUB = 1
  MUL = 2
  DIV = 3

class TestSeahorse(Account):
    owner: Pubkey
    display: i64
    
@instruction
def init_calculator(owner: Signer, calculator: Empty[TestSeahorse]):
    print("init calculator running")
    calculator = calculator.init(
        payer = owner,
        seeds = ['Calculator', owner]
    )
    calculator.owner = owner.key()
    
@instruction
def reset_calculator(owner: Signer, calculator: TestSeahorse):
    assert owner.key() == calculator.owner, "This is not your calculator"
    print(owner.key(), "is resetting", calculator.key())
    calculator.display = 0
    
@instruction
def do_operation(owner: Signer, calculator: TestSeahorse, op: Operation, num: i64):
    # Verify owner, like before
    print("do_operation running")
    assert owner.key() == calculator.owner, 'This is not your calculator!'


    if op == Operation.ADD:
        calculator.display += num
    elif op == Operation.SUB:
        calculator.display -= num
    elif op == Operation.MUL:
        calculator.display *= num
    elif op == Operation.DIV:
        calculator.display //= num