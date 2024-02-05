import MinimalWork from '../minimal/Work.vue';

describe('Minimal Work', () => {
  it('should render', () => {
    cy.mount(MinimalWork);
    cy.get('h2').should('have.text', 'Previous work');
  });

  it('should contain 2 work items', () => {
    cy.mount(MinimalWork);
    cy.get('section').find('h3').should('have.length', 2);
  });
});
