import MinimalInfo from '../minimal/Info.vue';

describe('Minimal Header', () => {
  it('playground', () => {
    cy.mount(MinimalInfo);
  });

  it('renders properly', () => {
    cy.mount(MinimalInfo);
    cy.get('section').should('exist');
  });
});
